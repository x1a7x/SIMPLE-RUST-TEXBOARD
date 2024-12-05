


![Screenshot 2024-12-05 123640](https://github.com/user-attachments/assets/4eff85d7-ecf5-473c-b9fb-4a6c9e186802)




![392625542-140f0ada-5792-479c-9842-fb10d20e749c](https://github.com/user-attachments/assets/407a4524-3aec-4ea1-80df-ae4d2171a329)








# Rust Simple Textboard

Rust  Textboard is a simple, lightweight web application implemented in **Rust** using the **Actix-Web** framework. It is designed as a message board for creating threads and replying to existing posts, with a clean and minimalistic front end. The application uses **Sled** as its data storage and **Askama** as the template engine, both of which are written in Rust and bring a number of unique benefits to the application.

## Table of Contents
- [Overview](#overview)
- [Technologies Used](#technologies-used)
- [Advantages of Sled and Askama](#advantages-of-sled-and-askama)
  - [Sled](#sled)
  - [Askama](#askama)
- [Setup Instructions](#setup-instructions)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Contributing](#contributing)

## Overview

Rust  Textboard provides users with the ability to create threads, reply to existing threads, and see the most recently updated threads at the top of the board. The application has been built with Actix-Web, a fast and secure web framework written in Rust. This ensures high performance and safety.

## Technologies Used
- **Rust**: The primary programming language used to build the application, providing safety and performance.
- **Actix-Web**: A powerful and asynchronous web framework for Rust.
- **Sled**: An embedded database for storing threads and replies, written entirely in Rust.
- **Askama**: A type-safe template engine for generating HTML from Rust data structures.

## Advantages of Sled and Askama

### Sled

**Sled** is an embedded database written entirely in Rust. It is often compared to databases like SQLite, but it is designed with different goals in mind, particularly focusing on reliability and simplicity.

**Advantages of Sled in Rust  Textboard:**
- **Rust Native:** Written in Rust, Sled benefits from Rust's guarantees of memory safety and concurrency. This makes it an excellent match for Rust applications that value performance and safety.
- **Embeddable:** Sled is an embedded database, meaning it can be bundled directly with the application. There's no need for a separate database server, which simplifies deployment and makes the application easy to run on a wide variety of platforms.
- **Crash Safety:** Sled is designed to be crash-safe and to recover from crashes without data loss, a vital feature for maintaining user content in the Textboard.
- **Simple API:** The API of Sled is minimalist, making it easy to interact with. This fits well with the requirements of the Textboard, where operations are straightforward (such as storing and retrieving threads and replies).

### Askama

**Askama** is a template engine for Rust inspired by Jinja (from Python). It is known for its simplicity, efficiency, and type-safety.

**Advantages of Askama in Rust  Textboard:**
- **Rust Native and Type-Safe:** Askama templates are compiled into Rust code, which means you get the same type safety and compile-time checks as the rest of your Rust code. This prevents many common runtime errors in templates.
- **Performance:** Since templates are compiled, the rendering is fast and efficient compared to many traditional template engines that interpret templates at runtime.
- **Integration with Rust:** Askama allows for the seamless integration of Rust structures and HTML templates. You can use Rust types directly in your templates, making it easy to keep your HTML in sync with the data model.
- **Simplicity and Familiar Syntax:** Askama's syntax is similar to other popular template engines like Jinja or Django, making it easy to use even for developers new to Rust.

## Setup Instructions

To get started with Rust  Textboard, follow these steps:

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/yourusername/Rust -Textboard.git
   cd Rust -Textboard
   ```

2. **Install Rust:**
   Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).

3. **Install Dependencies and Build the Project:**
   ```bash
   cargo build
   ```

4. **Run the Application:**
   ```bash
   cargo run
   ```

5. **Access the Application:**
   Open your browser and go to [http://localhost:8080](http://localhost:8080).

## Usage

- **Create a Thread:** Use the form on the main page to create a new thread by specifying a title and a message.
- **Reply to a Thread:** Navigate to an existing thread and use the reply form to add a response.
- **View Threads:** Threads are sorted so that the most recently updated thread appears at the top of the first page.

## Project Structure

The project is organized as follows:

```
.
├── Cargo.toml             # Dependency and project metadata
├── src
│   └── main.rs           # Main application entry point
├── templates              # HTML templates for Askama
│   ├── base.html         # Base template for common HTML structure
│   ├── homepage.html     # Template for the main page
│   └── thread.html       # Template for viewing individual threads
├── static                 # Static assets (CSS, JavaScript, images)
│   └── style.css         # Main stylesheet
└── uploads                # Directory for uploaded images (currently unused)
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request or create an issue if you find bugs or have suggestions for improvements.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/my-feature`).
3. Commit your changes (`git commit -am 'Add some feature'`).
4. Push to the branch (`git push origin feature/my-feature`).
5. Open a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

---

Rust  Textboard demonstrates the power of Rust in building efficient, safe, and performant web applications. With technologies like **Sled** and **Askama**, developers benefit from a unified, Rust-native stack that ensures both simplicity and reliability.

