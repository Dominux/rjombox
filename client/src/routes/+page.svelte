<script>
	import API from '$lib/api';

	const createRoom = async () => {
		await API.createRoom();
	};
</script>

<div>
	<h1>Welcum to Смертельная вечёринка</h1>

	<button on:click={createRoom}>Зайти в комнату</button>
</div>
