import { House } from "lucide-svelte";
import { writable } from "svelte/store";


export const tabs: { [key: string]: { title: string, component: any } } = {
	Home: {
		title: "Home",
		component: House,
	},
}
export const currentTab = writable<keyof typeof tabs>("Home");


