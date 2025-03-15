import { useEffect, useState } from "react";
import { IoMdSettings } from "react-icons/io";
import Modal from "./Modal";
import { load } from "@tauri-apps/plugin-store";
import { open } from "@tauri-apps/plugin-dialog";
import { useErrorHandler } from "../context/useErrorHandler";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "../context/ToastContext";

const SettingsButton = () => {
  const [isModalOpen, setModalOpen] = useState(false);
  const [launchArgs, setLaunchArgs] = useState<string>("");
  const [isSandbox, setIsSandbox] = useState(false);
  const [sandboxPath, setSandboxPath] = useState<string | null>("");
  const [isCnFirewall, setIsCnFirewall] = useState(false);
  const handleError = useErrorHandler();
  const { showToast } = useToast();

  const chineseTimezones = [
    "Asia/Shanghai",
    "Asia/Chongqing",
    "Asia/Harbin",
    "Asia/Urumqi",
    "Asia/Kashgar",
  ];

  const isChina = chineseTimezones.includes(Intl.DateTimeFormat().resolvedOptions().timeZone);

  useEffect(() => {
    const loadArgs = async () => {
      try {
        // Load the store.json file
        const store = await load("store.json");

        // Retrieve launch arguments
        const launchArgsData = await store.get<{ value: string }>("launchArgs");
        const sandboxPath = await store.get<{ value: string }>("sandboxPath");
        const isSandbox = await store.get<{ value: boolean }>("isSandbox");
        const isCnFirewall = await store.get<{ value: boolean }>("isCnFirewall");

        // Log retrieved values for debugging
        console.log("Launch Args:", launchArgsData);

        // Set state with fallback values
        setLaunchArgs(launchArgsData?.value ?? "");
        setIsSandbox(isSandbox?.value ?? true);
        setSandboxPath(sandboxPath?.value ?? null);
        setIsCnFirewall(isCnFirewall?.value ?? isChina);
      } catch (error) {
        console.error("Failed to load settings:", error);
        handleError(error);
      }
    };

    loadArgs();
  }, []);

  const toggleSandbox = () => {
    setIsSandbox(!isSandbox);
  };

  const toggleCnFirewall = () => {
    setIsCnFirewall(!isCnFirewall);
  }

  async function selectFile() {
    const file = await open({
      multiple: false,
      directory: false,
    });
    setSandboxPath(file);
  }

  const saveArgs = async () => {
    try {
      // Load the store.json file
      const store = await load("store.json");
      // Save the current arguments to the store
      await store.set("launchArgs", { value: launchArgs });
      await store.set("isSandbox", { value: isSandbox });
      await store.set("sandboxPath", { value: sandboxPath });
      await store.save();
      console.log("Launch arguments saved successfully.");
    } catch (error) {
      handleError(error);
      console.error("Failed to save launch arguments:", error);
    } finally {
      setModalOpen(false);
    }
  };

  // Sandboxie actions
  const permitPluginsFolder = async () => {
    try {
      await invoke("sandboxie_permit_plugins_folder");
      showToast("Sandboxie plugin folder permitted.", "alert-success");
    } catch (error) {
      showToast(
        `Error: ${error}`,
        "alert-error"
      );
      handleError(error);
    }
  };

  const blockCacheFolders = async () => {
    try {
      await invoke("sandboxie_block_cache_folders");
      showToast(
        "Access to main cache folders has been blocked.",
        "alert-success"
      );
    } catch (error) {
      showToast(
        `Error: ${error}`,
        "alert-error"
      );
      handleError(error);
    }
  };

  const revokePluginsFolder = async () => {
    try {
      await invoke("sandboxie_revoke_plugins_folder");
      showToast("Sandboxie plugin folder revoked.", "alert-success");
    } catch (error) {
      showToast(
        `Error: ${error}`,
        "alert-error"
      );
      handleError(error);
    }
  };

  const unblockCacheFolders = async () => {
    try {
      await invoke("sandboxie_unblock_cache_folders");
      showToast("Access to main cache folders unblocked.", "alert-success");
    } catch (error) {
      showToast(
        `Error: ${error}`,
        "alert-error"
      );
      handleError(error);
    }
  };

  const blockRegistry = async () => {
    try {
      await invoke("sandboxie_block_user_registry");
      showToast("User registry blocked.", "alert-success");
    } catch (error) {
      showToast(`Error: ${error}`, "alert-error");
      handleError(error);
    }
  };

  const unblockRegistry = async () => {
    try {
      await invoke("sandboxie_unblock_user_registry");
      showToast("User registry unblocked.", "alert-success");
    } catch (error) {
      showToast(`Error: ${error}`, "alert-error");
      handleError(error);
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
        <label className="form-control w-full">
          <div className="label">
            <span className="label-text">Sandboxie "Start.exe" path</span>
          </div>
          <input
            type="text"
            className="input w-full bg-base-200 hover:cursor-pointer border-disabled"
            onClick={selectFile}
            placeholder={sandboxPath || `Select Sandboxie "Start.exe" file...`}
            readOnly
          />
        </label>
        <div className="form-control">
          <label className="label cursor-pointer">
            <span className="label-text">Process Isolation</span>
            <input
              type="checkbox"
              className="toggle toggle-primary"
              checked={isSandbox}
              onChange={toggleSandbox}
            />
          </label>
          <label className="label cursor-pointer">
            <span className="label-text">绕过中国防火长城</span>
            <input
                type="checkbox"
                className="toggle toggle-primary"
                checked={isCnFirewall}
                onChange={toggleCnFirewall}
            />
          </label>
        </div>
        <div className="mt-4 flex flex-col gap-2">
          <p className="text-warning">
            The settings below require administrator access, make sure you are
            running as admin.
          </p>
          <div className=" grid grid-rows-2 grid-cols-3 gap-x-2">
            <button className="btn btn-sm btn-success" onClick={permitPluginsFolder}>
              Permit Plugins Folder
            </button>
            <button className="btn btn-sm btn-success" onClick={blockCacheFolders}>
              Block Cache Folders
            </button>
            <button className="btn btn-sm btn-success" onClick={blockRegistry}>
              Block Registry
            </button>
            <button className="btn btn-sm btn-error" onClick={revokePluginsFolder}>
              Revoke Plugins Folder
            </button>
            <button className="btn btn-sm btn-error" onClick={unblockCacheFolders}>
              Unblock Cache Folders
            </button>
            <button className="btn btn-sm btn-error" onClick={unblockRegistry}>
              Unblock Registry
            </button>
          </div>
        </div>
      </Modal>
    </div>
  );
};

export default SettingsButton;
