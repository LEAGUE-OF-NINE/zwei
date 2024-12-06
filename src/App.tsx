import "./App.css";
import StartLethe from "./components/StartLethe";
import UpdateLetheButton from "./components/UpdateLetheButton";
import UpdateLimbus from "./components/UpdateLimbus";
import ViewLetheFiles from "./components/ViewLetheFilesButton";

function App() {
  return (
    <main className="h-screen w-full flex flex-col items-center gap-8 p-4">
      <div className="container flex flex-col items-center">
        <img src="./Zwei.png" alt="Zwei: Your Shield" />
        <h1 className="text-2xl">Zwei - Your Shield</h1>
      </div>
      <UpdateLimbus />
      <div className="flex gap-2">
        <StartLethe />
        <UpdateLetheButton />
        <ViewLetheFiles />
      </div>
    </main>
  );
}

export default App;
