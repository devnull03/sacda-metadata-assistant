use candle_core::shape::Dim;
use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{Activation, Dropout, Embedding, LayerNorm, Module, VarBuilder};
use candle_nn::{embedding, layer_norm, linear};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct VisionConfig {
    depth: usize,
    hidden_act: String,
    hidden_size: usize,
    intermediate_size: usize,
    num_heads: usize,
    in_chans: usize,
    out_hidden_size: usize,
    patch_size: usize,
    spatial_merge_size: usize,
    spatial_patch_size: usize,
    window_size: usize,
    fullatt_block_indexes: Vec<usize>,
    tokens_per_second: usize,
    temporal_patch_size: usize,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct RopeScaling {
    #[serde(rename = "type")]
    scaling_type: String,
    mrope_section: Vec<usize>,
}

#[allow(dead_code)]
struct Qwen2Config {
    attention_dropout: f32,
    bos_token_id: usize,
    eos_token_id: usize,
    vision_start_token_id: usize,
    vision_end_token_id: usize,
    vision_token_id: usize,
    image_token_id: usize,
    video_token_id: usize,
    hidden_act: String,
    hidden_size: usize,
    initializer_range: f64,
    intermediate_size: usize,
    max_position_embeddings: usize,
    max_window_layers: usize,
    model_type: String,
    num_attention_heads: usize,
    num_hidden_layers: usize,
    num_key_value_heads: usize,
    rms_norm_eps: f64,
    rope_theta: f64,
    sliding_window: usize,
    tie_word_embeddings: bool,
    torch_dtype: String,
    use_cache: bool,
    use_sliding_window: bool,
    vision_config: VisionConfig,
    rope_scaling: RopeScaling,
    vocab_size: usize,
}

impl Default for Qwen2Config {
    fn default() -> Self {
        Self {
            attention_dropout: 0.0,
            bos_token_id: 151643,
            eos_token_id: 151645,
            vision_start_token_id: 151652,
            vision_end_token_id: 151653,
            vision_token_id: 151654,
            image_token_id: 151655,
            video_token_id: 151656,
            hidden_act: "silu".to_string(),
            hidden_size: 3584,
            initializer_range: 0.02,
            intermediate_size: 18944,
            max_position_embeddings: 128000,
            max_window_layers: 28,
            model_type: "qwen2_5_vl".to_string(),
            num_attention_heads: 28,
            num_hidden_layers: 28,
            num_key_value_heads: 4,
            rms_norm_eps: 1e-6,
            rope_theta: 1000000.0,
            sliding_window: 32768,
            tie_word_embeddings: false,
            torch_dtype: "bfloat16".to_string(),
            use_cache: true,
            use_sliding_window: false,
            vision_config: VisionConfig {
                depth: 32,
                hidden_act: "silu".to_string(),
                hidden_size: 1280,
                intermediate_size: 3420,
                num_heads: 16,
                in_chans: 3,
                out_hidden_size: 3584,
                patch_size: 14,
                spatial_merge_size: 2,
                spatial_patch_size: 14,
                window_size: 112,
                fullatt_block_indexes: vec![7, 15, 23, 31],
                tokens_per_second: 2,
                temporal_patch_size: 2,
            },
            rope_scaling: RopeScaling {
                scaling_type: "mrope".to_string(),
                mrope_section: vec![16, 24, 24],
            },
            vocab_size: 152064,
        }
    }
}


fn cumsum_2d(mask: &Tensor, dim: u8, device: &Device) -> Result<Tensor> {
    let mask = mask.to_vec2::<u8>()?;

    let rows = mask.len();
    let cols = mask[0].len();

    let mut result = mask.clone();

    match dim {
        0 => {
            // Cumulative sum along rows
            for i in 0..rows {
                for j in 1..cols {
                    result[i][j] += result[i][j - 1];
                }
            }
        }
        1 => {
            // Cumulative sum along columns
            for j in 0..cols {
                for i in 1..rows {
                    result[i][j] += result[i - 1][j];
                }
            }
        }
        _ => panic!("Dimension not supported"),
    }

    let result = Tensor::new(result, &device)?;

    Ok(result)
}

pub fn create_position_ids_from_input_ids(
    input_ids: &Tensor,
    padding_idx: u32,
    past_key_values_length: u8,
) -> Result<Tensor> {
    let mask = input_ids.ne(padding_idx)?;
    let incremental_indices = cumsum_2d(&mask, 0, input_ids.device())?;

    let incremental_indices = incremental_indices
        .broadcast_add(&Tensor::new(&[past_key_values_length], input_ids.device())?)?;

    Ok(incremental_indices)
}

pub struct Qwen2Embeddings {
    word_embeddings: Embedding,
    position_embeddings: Option<Embedding>,
    layer_norm: LayerNorm,
    dropout: Dropout,
}

impl Qwen2Embeddings {
    pub fn load(vb: VarBuilder, config: &Qwen2Config) -> Result<Self> {
        let word_embeddings = embedding(
            config.vocab_size,
            config.hidden_size,
            vb.pp("word_embeddings"),
        )?;

        let position_embeddings = if !config.tie_word_embeddings {
            Some(embedding(
                config.max_position_embeddings,
                config.hidden_size,
                vb.pp("position_embeddings"),
            )?)
        } else {
            None
        };

        let layer_norm = layer_norm(
            config.hidden_size,
            config.rms_norm_eps,
            vb.pp("LayerNorm"),
        )?;

        let dropout = Dropout::new(config.attention_dropout);

        Ok(Self {
            word_embeddings,
            position_embeddings,
            layer_norm,
            dropout,
        })
    }

    pub fn forward(
        &self,
        input_ids: &Tensor,
        position_ids: Option<&Tensor>,
        inputs_embeds: Option<&Tensor>,
    ) -> Result<Tensor> {
        let inputs_embeds = match inputs_embeds {
            Some(embeds) => embeds.to_owned(),
            None => self.word_embeddings.forward(input_ids)?,
        };

        let mut embeddings = inputs_embeds;

        if let Some(position_embeddings) = &self.position_embeddings {
            if let Some(position_ids) = position_ids {
                embeddings = embeddings.broadcast_add(&position_embeddings.forward(position_ids)?)?;
            }
        }

        let embeddings = self.layer_norm.forward(&embeddings)?;
        let embeddings = self.dropout.forward(&embeddings, false)?;

        Ok(embeddings)
    }
}
