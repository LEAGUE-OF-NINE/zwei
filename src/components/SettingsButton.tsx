import { useEffect, useState } from "react";
import { IoMdSettings } from "react-icons/io";
import Modal from "./Modal";
import { load } from "@tauri-apps/plugin-store";
import { open } from "@tauri-apps/plugin-dialog";

const SettingsButton = () => {
  const [isModalOpen, setModalOpen] = useState(false);
  const [launchArgs, setLaunchArgs] = useState<string>("");
  const [sandbox, setSandbox] = useState<boolean>(false);
  const [sandboxPath, setSandboxPath] = useState<string | null>("");

  async function selectFile() {
    const file = await open({
      multiple: false,
      directory: false,
    });
    setSandboxPath(file);
  }

  useEffect(() => {
    const loadArgs = async () => {
      try {
        // Load the store.json file
        const store = await load("store.json");

        // Retrieve the launch arguments
        let launchArgs = await store.get<{ value: string }>("launchArgs");
        if (launchArgs === undefined) {
          launchArgs = { value: "" };
        }
        let sandbox = await store.get<{ value: boolean }>("sandbox");
        if (sandbox === undefined) {
          sandbox = { value: false };
        }
        let sandboxPath = await store.get<{ value: string }>("sandboxPath");
        if (sandboxPath === undefined) {
          sandboxPath = { value: "C:\\Program Files\\Sandboxie\\Start.exe" };
        }

        if (launchArgs) {
          setLaunchArgs(launchArgs.value); // Update the state with the loaded arguments
        }
        if (sandbox) {
          setSandbox(sandbox.value);
        }
        if (sandboxPath) {
          setSandboxPath(sandboxPath.value);
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
      await store.set("launchArgs", { value: launchArgs });
      await store.set("sandbox", { value: sandbox });
      await store.set("sandboxPath", { value: sandboxPath });
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
              value={launchArgs}
              onChange={(e) => setLaunchArgs(e.target.value)}
            />
          </div>

          <label className="form-control w-full">
            <div className="label">
              <span className="label-text">Sandboxie "Start.exe" path</span>
            </div>
            <input
              type="text"
              className="input w-full bg-base-200 hover:cursor-pointer border-disabled"
              onClick={selectFile}
              placeholder={
                sandboxPath || `Select Sandboxie "Start.exe" file...`
              }
              readOnly
            />
          </label>

          <label className="label cursor-pointer">
            <span className="label-text">Enable Sandboxing</span>
            <input
              type="checkbox"
              className="toggle toggle-primary"
              checked={sandbox}
              onChange={() => setSandbox((prev) => !prev)}
            />
          </label>
        </div>
      </Modal>
    </div>
  );
};

export default SettingsButton;
