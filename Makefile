run_server:
	cd server && cargo run --features dev || cd ../

run_client:
	cd client && pnpm run dev || cd ../
