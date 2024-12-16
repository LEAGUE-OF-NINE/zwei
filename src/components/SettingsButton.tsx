import { useEffect, useState } from "react";
import { IoMdSettings } from "react-icons/io";
import Modal from "./Modal";
import { load } from "@tauri-apps/plugin-store";
import { useErrorHandler } from "../context/useErrorHandler";

const SettingsButton = () => {
  const [isModalOpen, setModalOpen] = useState(false);
  const [launchArgs, setLaunchArgs] = useState<string>("");
  const handleError = useErrorHandler();

  useEffect(() => {
    const loadArgs = async () => {
      try {
        // Load the store.json file
        const store = await load("store.json");

        // Retrieve launch arguments
        const launchArgsData = await store.get<{ value: string }>("launchArgs");

        // Log retrieved values for debugging
        console.log("Launch Args:", launchArgsData);

        // Set state with fallback values
        setLaunchArgs(launchArgsData?.value ?? "");
      } catch (error) {
        console.error("Failed to load settings:", error);
        handleError(error);
      }
    };

    loadArgs();
  }, []);

  const saveArgs = async () => {
    try {
      // Load the store.json file
      const store = await load("store.json");
      // Save the current arguments to the store
      await store.set("launchArgs", { value: launchArgs });
      await store.save();
      console.log("Launch arguments saved successfully.");
    } catch (error) {
      handleError(error);
      console.error("Failed to save launch arguments:", error);
    } finally {
      setModalOpen(false);
    }
  };

  return (
    <div>
      <button
        className="btn btn-primary rounded-tl-none rounded-bl-none"
        onClick={() => setModalOpen(true)}
      >
        <IoMdSettings size={22} />
      </button>

      <Modal
        isOpen={isModalOpen}
        title="Settings"
        onClose={() => setModalOpen(false)}
        actions={
          <button className="btn btn-primary" onClick={saveArgs}>
            Save
          </button>
        }
      >
        <div className="space-y-4">
          <div>
            <label className="label">
              <span className="label-text">Launch Arguments</span>
            </label>
            <input
              type="text"
              className="input input-bordered w-full"
              value={launchArgs}
              onChange={(e) => setLaunchArgs(e.target.value)}
            />
          </div>
        </div>
      </Modal>
    </div>
  );
};

export default SettingsButton;
