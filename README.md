# 🦀 WebAssembly (Wasm) Demo with Rust

This is a simple **WebAssembly (Wasm) demo** that compiles a Rust function to WebAssembly and runs it in the browser.

---

## 🚀 Getting Started

Follow these steps to set up and run the project.

---

### **1️⃣ Prerequisites**

Ensure you have the following tools installed:

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **wasm-pack**: For compiling Rust to WebAssembly
- **Local Server**: Use Node.js or any other local server tool

---

### **2️⃣ Installation Steps**

1. **Install Rust (if not already installed):**

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Add WebAssembly target:**

   ```sh
   rustup target add wasm32-unknown-unknown
   ```

3. **Install wasm-pack:**

   ```sh
   cargo install wasm-pack
   ```

4. **Compile Rust into WebAssembly:**

   ```sh
   wasm-pack build --target web
   ```

5. **Start a local server:**

   ```sh
   npx serve
   ```

6. **Rebuild the Wasm package after making changes:**

   ```sh
   wasm-pack build --target web
   ```

---

### **3️⃣ Running the Demo**

- Open the `index.html` file in your browser to see the WebAssembly demo in action.
- Make changes to the Rust code in `src/lib.rs` and rebuild using the steps above.

---

### **📂 Project Structure**

- `src/`: Contains the Rust source code
- `pkg/`: Generated WebAssembly files
- `index.html`: Entry point for the browser

---
