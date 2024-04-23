import {currentMonitor, LogicalSize, PhysicalPosition, PhysicalSize, availableMonitors} from "@tauri-apps/api/window";
import { appWindow } from '@tauri-apps/api/window';

let appHeight = 1200;
let appWidth = 600;

export async function repositionApp() {
	const monitor = await currentMonitor();
	const { size, position, scaleFactor } = monitor;
	let newWidth = Math.floor(size.width * 0.66 / scaleFactor);
	if (newWidth < 400) {
		newWidth = 400;
	}
	appWidth = newWidth;
	const x = position.x + 500;
	const y = position.y;
	await appWindow.setSize(new PhysicalSize(appWidth, appHeight));
	await appWindow.setPosition(new PhysicalPosition(-x,-y));
}

export async function setAppHeight(height: number) {
	appHeight = height;
	await appWindow.setSize(new LogicalSize(appWidth, appHeight));
}
