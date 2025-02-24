import { House, Sheet, TestTube2 } from "lucide-svelte";
import { writable } from "svelte/store";


export const tabs: { [key: string]: { title: string, component: any, path: string } } = {
	Home: {
		title: "Home",
		path: "/",
		component: House,
	},
	Excel: {
		title: "Excel",
		path: "/excel",
		component: Sheet,
	},
	Testing: {
		title: "Testing",
		path: "/test",
		component: TestTube2,
	}
}
export const currentTab = writable<keyof typeof tabs>("Home");
