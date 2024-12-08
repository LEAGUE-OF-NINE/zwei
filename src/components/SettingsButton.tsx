import { useState } from "react";
import { IoMdSettings } from "react-icons/io";
import Modal from "./Modal"; // Ensure you import the reusable Modal component

const SettingsButton = () => {
  const [isModalOpen, setModalOpen] = useState(false);

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
      >
        <div className="space-y-4">
          <div>
            <label className="label">
              <span className="label-text">Launch Arguments</span>
            </label>
            <input type="text" className="input input-bordered w-full" />
          </div>
          <div className="form-control w-52">
            <label className="label cursor-pointer">
              <span className="label-text">Enable Sandboxing</span>
              <input
                type="checkbox"
                className="toggle toggle-primary"
              />
            </label>
          </div>
        </div>
      </Modal>
    </div>
  );
};

export default SettingsButton;
