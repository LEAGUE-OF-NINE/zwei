import { useEffect, useState } from "react";
import { IoMdSettings } from "react-icons/io";
import Modal from "./Modal"; // Ensure you import the reusable Modal component
import { load } from "@tauri-apps/plugin-store";

const SettingsButton = () => {
  const [isModalOpen, setModalOpen] = useState(false);
  const [args, setArgs] = useState<string>("");

  useEffect(() => {
    const loadArgs = async () => {
      try {
        // Load the store.json file
        const store = await load("store.json");

        // Retrieve the launch arguments
        const launchArgs = await store.get<{ value: string }>("launchArgs");

        if (launchArgs) {
          setArgs(launchArgs.value); // Update the state with the loaded arguments
        }
      } catch (error) {
        console.error("Failed to load launch arguments:", error);
      }
    };

    loadArgs();
  }, []);

  const saveArgs = async () => {
    try {
      // Load the store.json file
      const store = await load("store.json");
      // Save the current arguments to the store
      await store.set("launchArgs", { value: args });
      await store.save();
      console.log("Launch arguments saved successfully.");
    } catch (error) {
      console.error("Failed to save launch arguments:", error);
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
              value={args}
              onChange={(e) => setArgs(e.target.value)}
            />
          </div>
          <div className="form-control w-52">
            <label className="label cursor-pointer">
              <span className="label-text">Enable Sandboxing</span>
              <input type="checkbox" className="toggle toggle-primary" />
            </label>
          </div>
        </div>
      </Modal>
    </div>
  );
};

export default SettingsButton;
