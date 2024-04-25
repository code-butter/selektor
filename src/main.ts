import './app.css'
import App from './App.svelte'
import {repositionApp} from "./lib/utils";


const app = new App({
  target: document.getElementById('app'),
})

repositionApp()
window.setTimeout(repositionApp) // Call it again just to make sure it gets repositioned

export default app
