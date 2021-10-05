import axios from 'axios';


const axios_base = axios.create({
  //baseURL: "http://192.168.1.4:8080",
  //baseURL: "http://localhost:8080",
  timeout: 5000,
  headers: {
    "Content-type": "application/json"
  },

  
});

const axios_polling = axios.create({
  //baseURL: "http://192.168.1.4:8080",
  //baseURL: "http://localhost:8080",
  //timeout: 1000,
  headers: {
    "Content-type": "application/json"
  }
});

export {
  axios_base,
  axios_polling
}
