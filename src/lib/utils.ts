import {currentMonitor, LogicalSize, PhysicalPosition, PhysicalSize, availableMonitors} from "@tauri-apps/api/window";
import { appWindow } from '@tauri-apps/api/window';

let appHeight = 1;
let appWidth = 1;

export async function repositionApp() {
	const monitor = await currentMonitor();
	const { size, position, scaleFactor } = monitor;
	let newWidth = Math.floor(size.width * 0.66 / scaleFactor);
	if (newWidth < 400) {
		newWidth = 400;
	}
	appWidth = newWidth;
	const x = Math.floor(position.x + (newWidth / 2));
	const y = Math.floor(position.y + (size.height / 3));
	await appWindow.setSize(new PhysicalSize(appWidth, appHeight));
	await appWindow.setPosition(new PhysicalPosition(x,y));
}

export async function setAppHeight(height: number) {
	appHeight = Math.floor(height);
	if (appHeight < 1) {
		appHeight = 1;
	}
	await appWindow.setSize(new LogicalSize(appWidth, appHeight));
}
