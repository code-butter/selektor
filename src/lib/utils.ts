import {currentMonitor, LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize} from "@tauri-apps/api/window";
import { appWindow } from '@tauri-apps/api/window';

let appHeight = 1200;
let appWidth = 600;

export async function repositionApp() {
	const monitor = await currentMonitor();
	const { size, position } = monitor;
	let newWidth = Math.floor(size.width * 0.3);
	if (newWidth < 400) {
		newWidth = 400;
	}
	appWidth = newWidth;
	const x = position.x;
	const y = position.y;
	await appWindow.setPosition(new LogicalPosition(1,1));
	await appWindow.setSize(new LogicalSize(appWidth, appHeight));

}

export async function setAppHeight(height: number) {
	appHeight = height;
	await appWindow.setSize(new LogicalSize(appWidth, appHeight));
}
