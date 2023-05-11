# Build and Run for Windows

Prerequisites.
1. Install: Node.js
2. Install: npm
3. Run: npm install
4. Run: cargo install wasm-pack

Build and Run Dev.
1. wasm-pack build --target web
2. cd .\www
3. npm run build
4. npm run dev
5. Navigate to http://localhost:8080/

Build and Run Prod.
1. wasm-pack build --target web
2. npm run build
3. npm run start
4. Navigate to http://localhost:3000/
