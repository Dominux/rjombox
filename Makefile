run_server:
	cd server && cargo run || cd ../

run_client:
	cd client && pnpm run dev || cd ../
