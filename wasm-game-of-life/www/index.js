import * as wasm from "wasm-game-of-life";
import axios from 'axios';


wasm.log_to_console("text");
console.log(wasm.fib(3));



function loadStringFromWebsite(url) {

  return axios.get(url)
    .then(response => {
      
      wasm.log_to_console(response.data);
      return response.data;
    })
    .catch(error => {
      console.error('Error fetching data:', error);
    });
}

let url = "https://code.jquery.com/jquery-3.7.0.js";
loadStringFromWebsite(url);
