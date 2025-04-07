# Gengo

Gengo is a full-stack **language learning platform** that helps users improve their speaking skills through real-time speech recognition and feedback. It provides a web application where users can **record their speech**, get automated analysis of their **grammar and pronunciation**, and receive personalized feedback. By combining a Rust backend and a Rust WebAssembly frontend, Gengo delivers a seamless experience for practicing language speaking and tracking progress.

## Projected to have these features: 

- **User Authentication & Profiles:** Users can create accounts, log in with secure JWT-based authentication, and manage personal profiles (e.g., username, language preferences, etc.). Session management ensures only authorized access to the app’s features.
- **Speech Recording & Playback:** The web app allows users to record their voice through the browser (using the device microphone). Recorded audio is captured and sent to the backend for processing. Users can playback their recordings to hear themselves.
- **Speech Recognition (Transcription):** Gengo converts recorded audio into text. This speech-to-text functionality lets the system understand what the user said. (Under the hood, this may integrate with a speech recognition engine or API.)
- **Grammar Analysis:** The transcribed text is analyzed for grammatical accuracy. Gengo identifies grammar mistakes or weaknesses in the spoken sentences and can highlight errors or suggest corrections.
- **Pronunciation Analysis:** Gengo evaluates the user’s pronunciation by comparing the audio to expected pronunciation patterns. It can detect mispronounced words or sounds and provide feedback on how to improve pronunciation.
- **Feedback and Scoring:** After each practice session, users receive detailed **feedback**. The feedback component of the app displays grammar corrections, pronunciation tips, and an overall score or rating for the session. This helps users understand their performance and track improvement over time.
- **Dashboard & Practice Interface:** A user-friendly dashboard summarizes past sessions and progress. The **Practice** page guides users through speaking exercises. During practice, users can see prompts (if provided), record their speech, and submit it for analysis. The results are then shown on a **Feedback** page, which the user can review.
- **Real-time Processing (WebSockets):** *(Planned)* Gengo’s architecture supports real-time feedback via WebSockets. The backend can stream analysis results to the frontend as the user speaks. This means in future versions, users might get live feedback (e.g., a running transcript or alerts) while still recording.
- **Responsive Single-Page Application:** The frontend is built as a responsive SPA (Single-Page Application). Users get a smooth, app-like experience in the browser with dynamic content updates without full page reloads. The interface is designed to be intuitive for language practice, with components like a navigation bar, recording controls, and feedback display.

## Technologies Used

- **Rust (Backend – Rocket):** The backend server is implemented in Rust using the [Rocket](https://rocket.rs) web framework. Rocket handles HTTP requests for API endpoints (user auth, session data, feedback) and serves the application content. It provides a robust foundation for routing, data validation, and JSON serialization (using Rocket’s JSON support via Serde).
- **Rust (Frontend – Leptos):** The frontend is a client-side web application written in Rust using the [Leptos](https://github.com/leptos-rs/leptos) framework. Leptos enables building reactive web UIs in Rust, which are compiled to WebAssembly. This allows the entire application (frontend and backend) to be written in one language (Rust).
- **WebAssembly (WASM):** The Leptos frontend is compiled to WebAssembly and runs in the browser. This provides near-native performance for the client-side app and enables access to Web APIs for features like audio recording.
- **MediaRecorder API (via web-sys):** The frontend uses the browser’s Media Devices and MediaRecorder APIs (through Rust’s `web-sys` and `js-sys` crates) to access the microphone and record audio. This recorded audio is then either processed in-browser or sent to the backend for analysis.
- **SQLx and SQLite (Database):** Gengo uses **SQLx** (an async Rust SQL toolkit) with an **SQLite** database for data persistence. The database stores user accounts, session records, and possibly logs of feedback. By default, the app is configured to use an in-memory SQLite database for development (for quick setup). It can be pointed to a file-based SQLite database for persistent storage.
- **JSON Serialization (Serde):** Data exchanged between the frontend and backend (such as user info, transcripts, feedback results) is encoded in JSON. The Serde library is used on the backend for serializing and deserializing JSON payloads in API requests/responses.
- **Asynchronous Rust (Tokio):** The backend is asynchronous (thanks to Rocket’s async support and Tokio). This allows handling of concurrent requests (multiple users or multiple processing tasks) efficiently. It’s crucial for potentially processing audio in the background without blocking the server.
- **JWT for Authentication:** JSON Web Tokens are used to authenticate users and protect API routes. On login, the backend issues a JWT signed with a secret (configurable via environment variable). The frontend stores this token and includes it in subsequent requests to secured endpoints. This ensures that only authorized users can access their data.
- **Rocket WebSocket (rocket_ws):** The backend includes the `rocket_ws` crate to enable WebSocket endpoints. This is used (or planned to be used) for streaming scenarios, such as sending live transcription or feedback to the frontend while the user is speaking.
- **Trunk & Tooling:** [Trunk](https://trunkrs.dev) is used as the build and development server for the frontend. It compiles the Rust WASM code, watches for changes in development, and serves the static files. Other tooling includes **Cargo** (Rust’s package manager and build tool) for managing the workspace, and **dotenv** for loading environment variables from a `.env` file during development.

## Installation:

Follow these steps to set up the Gengo project on your local machine:

1. **Prerequisites:**  
   - **Rust toolchain:** Install Rust (via [rustup](https://rustup.rs)) if you haven’t already. Gengo targets Rust 2021 edition and does not require nightly; the stable toolchain is sufficient.  
   - **Trunk:** Install Trunk to build and serve the frontend. You can install it with Cargo:  
     ```bash
     cargo install trunk
     ```  
     Also, ensure you have the WebAssembly target installed for Rust:  
     ```bash
     rustup target add wasm32-unknown-unknown
     ```  
     *Note:* Trunk will use this target to compile the frontend to WASM.
   - **SQLite (optional):** SQLite is used via a file or in-memory. You don’t need to set up a database server. However, ensure that the application can create a SQLite file if using file mode. (By default, it uses in-memory DB, so this is optional.)
2. **Clone the Repository:**  
   Clone the Gengo repository from GitHub to your local system:  
   ```bash
   git clone https://github.com/Utkarsh-upadhyay9/Gengo.git
   ```  
   This will create a directory `Gengo` with the project files.
3. **Configure Environment (optional):**  
   The backend configuration is managed via environment variables (loaded from a `.env` file in `gengo/backend/`). The project already provides a sample development `.env` with defaults. You can review or edit `gengo/backend/.env` if needed:  
   - `DATABASE_URL` – SQLite connection string. By default it’s set to use an in-memory database (`sqlite::memory:`). If you want to persist data, you can change this to a file path, e.g. `sqlite:gengo.db` to use a file database.  
   - `JWT_SECRET` – Secret key for signing JWT tokens (for authentication). A default (`dev-secret-change-in-production`) is provided. It’s recommended to change this in a production setting.  
   Other Rocket configurations (address, port, etc.) are set in `gengo/backend/Rocket.toml` (default port 8000 is used).
4. **Build Frontend Assets:**  
   Navigate to the frontend directory and build the WebAssembly bundle using Trunk:  
   ```bash
   cd Gengo/gengo/frontend
   trunk build --release
   ```  
   This will compile the Leptos frontend and output static files (HTML, JS, WASM, CSS) into the `dist` directory. The `index.html` and `pkg` (package with `.wasm` and JS files) will be generated. (In development, you could use `trunk serve` to auto-reload, but for deployment we build the static files.)
5. **Run the Backend Server:**  
   Open another terminal and navigate to the backend directory to run the Rocket server:  
   ```bash
   cd Gengo/gengo/backend
   cargo run
   ```  
   This will compile and launch the backend API server. By default it listens on `127.0.0.1:8000` (as configured in Rocket.toml). When the server starts, you should see Rocket’s launch message indicating it’s running.
6. **Serve the Frontend:**  
   There are two ways to serve the frontend:  
   **a. Using Rocket to serve static files (recommended):** If you built the frontend in step 4, the output files can be served by the Rocket backend. Ensure the compiled `dist` files are placed where Rocket can find them (the repository is set up with a `backend/static/` folder). In this project, the `backend/static` directory already contains an `index.html` and other assets; you can update those with the latest build. The Rocket server is configured to serve the static folder, so your frontend should be accessible via the same address as the backend.  
   *In development*, you might simply copy the `dist` output into the `backend/static` folder:  
   ```bash
   cp -r ../frontend/dist/* ./static/
   ```  
   (This step might not be necessary if the repository is structured to output there directly or already contains the latest build.) Once copied, the Rocket server at `http://127.0.0.1:8000` will serve the UI.  
   **b. Using Trunk’s development server:** Alternatively, for quick testing, you can run Trunk’s built-in server:  
   ```bash
   trunk serve
   ```  
   This will host the frontend on `http://127.0.0.1:8080` and watch for code changes. However, if you use this method, note that API calls (to `127.0.0.1:8000`) will be cross-origin. You may need to enable CORS in the backend or proxy API requests. The recommended approach is (a) for an integrated experience.
7. **Finalize Setup:**  
   Once the backend is running (and serving static files or your frontend is being served), you can access Gengo in your web browser.

## Projected usage after development:

**Access the Application:** Open your browser and navigate to the URL where Gengo is served. For example: 

- If using the Rocket server to serve the UI, go to **`http://127.0.0.1:8000`**.  
- If using `trunk serve` (frontend dev server), go to **`http://127.0.0.1:8080`** (make sure the backend is still running on 8000 for API).

You should see the Gengo web application load in your browser. 

**Create an Account:** Start by signing up for a new account. Click on the **Register/Sign Up** link (or navigate to the Sign Up page) and fill in your details. Gengo will create a new user in its database. After registering, log in with your credentials. (If a default/demo account exists, it would be mentioned here, but generally each user creates their own.)

**Using the Dashboard:** After logging in, you’ll land on the **Dashboard**. The dashboard provides an overview of your activity. Since you’re new, it might be empty or show a welcome message. As you use Gengo, the dashboard will display information like the number of practice sessions completed, your average scores, and any other relevant stats or learning progress.

**Practicing Speech:** Navigate to the **Practice** page (often accessible via a navigation bar or a dashboard link). Here’s how to conduct a practice session:
1. **Start Recording:** You’ll see a microphone or record button. Click this to start a new recording. Your browser may prompt you to allow microphone access – make sure to allow it so Gengo can capture your voice.
2. **Speak:** Once recording starts, speak clearly into your microphone. You might talk about a given prompt or freely converse in the language you’re practicing. (If Gengo provides a text prompt or topic, it will be displayed on this page for you to follow.)
3. **Stop Recording:** Click the stop button when you’ve finished speaking. The recorded audio will then be saved for analysis. Depending on the implementation, Gengo may either immediately start processing or wait for you to submit.
4. **Submit for Analysis:** If required, click on a **Submit** or **Analyze** button. The recorded audio is sent to the server. Gengo will perform speech-to-text transcription, grammar checking, and pronunciation analysis on the backend.
5. **View Feedback:** After a brief processing time, the app will display the results on the **Feedback** page (or below the recording controls). Here you will see a transcript of what you said, with any grammatical errors highlighted or corrected. You’ll also get notes on pronunciation – for example, words that might have been mispronounced could be flagged with suggestions on correct pronunciation. There may also be a score or rating for your session, giving you a quantitative measure of your performance.

**Reviewing Feedback:** Take time to review the feedback. Gengo’s feedback page is designed to be clear and helpful:
- Grammar feedback might show your spoken sentence with corrections (e.g., missing articles or verb tense issues) annotated. There could be explanations for each correction.
- Pronunciation feedback might list certain phonetic sounds or specific words that need improvement. For example, “Pronounce **“th”** sound more softly in the word **“the”**.”
- Overall assessment could be a score, percentage, or level (e.g., “Fluency: 8/10, Pronunciation: B grade, Grammar: A grade”). Use this to track progress over time.

You can also **listen to your recording** if the app provides a playback feature. This lets you hear yourself and correlate the feedback with your speaking.

**Iterate and Improve:** From the feedback page, you can usually start another practice or go back to the practice interface. It’s encouraged to practice regularly. Over time, you should see improvements on your dashboard metrics and feel more confident in speaking.

**Other Features:** Explore other sections of the app:
- **Profile Settings:** Update your profile information, or change settings like target language, difficulty level, etc., if such options exist.
- **Feedback History:** Gengo may keep a history of past sessions. If so, you can revisit previous feedback to see how you’ve improved or to retry specific exercises.
- **Logout/Login:** Standard authentication controls for security.

Because Gengo runs locally in this setup, all your data is stored in the local SQLite database. If you want to reset the app (clear data), you can stop the server, delete the SQLite file (if one was used) or simply restart if using in-memory DB (which resets data on each run).

*Note:* Gengo is in active development. Some features (like real-time feedback during recording, or advanced analysis) might be experimental or planned for future updates. You might encounter placeholder components in the UI. Check the repository updates for new features and improvements regularly.

## Contributing

Contributions to Gengo are welcome! If you’d like to improve the project or fix issues, please follow these guidelines:

- **Fork the Repository:** Click the “Fork” button on GitHub to create your own fork of the project.
- **Create a Branch:** In your fork, create a new branch for your feature or bugfix. For example: `feature/improve-feedback-ui` or `bugfix/fix-audio-upload`.
- **Make Changes:** Develop your feature or fix in your branch. Ensure that the code builds and that all existing functionality remains intact. If you add a new feature, try to include relevant tests or update documentation as needed.
- **Test Locally:** Run both the backend and frontend locally to test your changes in a full-stack environment. Make sure nothing is broken, and the new changes work as expected.
- **Commit and Push:** Commit your changes with clear commit messages. Push your branch to your GitHub fork.
- **Open a Pull Request:** Go to the original Gengo repository and open a Pull Request from your fork’s branch. Describe your changes in the PR template, including why the change is beneficial.
- **Code Review:** Engage in the code review process by responding to any feedback or questions on your PR. The maintainer (or other contributors) may suggest improvements.
- **Merge:** Once approved, your PR will be merged into the main repository.

You can also contribute by **opening issues**. If you find a bug or have an idea for a new feature, please open an issue on GitHub. Provide as much detail as possible (steps to reproduce, expected vs actual behavior, screenshots if applicable, etc.).

**Development Notes:** The project is organized as a Rust workspace with two main members: the backend (`gengo-backend`) and the frontend (`gengo-frontend`). Familiarizing yourself with Rust, Rocket, and Leptos will help in contributing. The file `structure` in the repository root outlines the project structure in detail, which can be useful for navigation. New contributors might start with smaller tasks like adjusting UI components (in `frontend/src/components`) or writing additional documentation.

We appreciate any form of contribution – from bug reports and feature requests to code improvements and documentation. By contributing, you help make Gengo better for everyone interested in practicing languages!

## License

*Currently, no license is specified for Gengo.* This means that the project is **not officially open-source at this time**. All rights are reserved by the author. If you wish to use or adapt Gengo’s code for your own purposes, you should contact the author for permission. (In the future, a license may be added to clarify usage rights – keep an eye on the repository for updates.)

## Contact Information

This project is maintained by **Utkarsh Upadhyay**. If you have any questions, suggestions, or inquiries regarding Gengo:

- **GitHub:** You can reach out by creating an issue or discussion in the [Gengo repository](https://github.com/Utkarsh-upadhyay9/Gengo). For direct contact, you may also mention the maintainer (GitHub username: `Utkarsh-upadhyay9`) in an issue or pull request.
- **Email/LinkedIn:** You can find me on [LinkedIn](https://www.linkedin.com/in/utkarsh-upadhyay-171bb718) for a professional connection or contact. *(If you prefer email, uu10@msstate.edu)*

Feel free to get in touch – whether it’s for support, to share feedback on your experience with Gengo, or to discuss collaboration. We’re excited to help language learners and appreciate your interest in the project! Happy learning and speaking with Gengo!
