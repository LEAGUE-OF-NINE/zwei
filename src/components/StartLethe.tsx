import { useEffect, useState } from "react";
import { useErrorHandler } from "../context/useErrorHandler";
import { open } from "@tauri-apps/plugin-shell";
import { listen } from "@tauri-apps/api/event";

const StartLethe = () => {
  const handleError = useErrorHandler();
  const [message, setMessage] = useState<string>("");

  useEffect(() => {
    const unlisten = listen<string>("launch-status", (event) => {
      console.log(`Received event: ${event.payload}`);
      setMessage(event.payload || "");
    });

    return () => {
      unlisten.then((unsub) => unsub());
    };
  }, []);

  async function startLethe() {
    try {
      await open("https://lethelc.site/?zwei");
      setMessage("Waiting for Site Launch signal...")
    } catch (error) {
      console.error("Failed to Start lethe:", error);
      handleError(error);
    }
  }

  return (
    <>
      <button
        className="btn btn-primary w-full max-w-sm text-xl font-semibold rounded-tr-none rounded-br-none"
        onClick={startLethe}
      >
        {message ? message : "Launch"}
      </button>
    </>
  );
};

export default StartLethe;
