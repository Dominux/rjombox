import axios from 'axios';

const API = {};

API.createRoom = async () => {
	await axios.get('http://localhost:26197');
};

export default API;
