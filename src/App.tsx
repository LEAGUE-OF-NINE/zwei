import "./App.css";
import CloneLimbus from "./components/CloneLimbus";

function App() {
  return (
    <main className="h-screen w-full flex flex-col items-center gap-8 p-4">
      <div className="container flex flex-col items-center">
        <img src="./Zwei.png" alt="Zwei: Your Shield" />
        <h1 className="text-2xl">Zwei - Your Shield</h1>
      </div>

      <CloneLimbus />
    </main>
  );
}

export default App;
