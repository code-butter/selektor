import './app.css'
import App from './App.svelte'
import {repositionApp} from "./lib/utils";


const app = new App({
  target: document.getElementById('app'),
})

repositionApp();

export default app
