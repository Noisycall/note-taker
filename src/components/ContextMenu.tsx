// src/components/ContextMenu.tsx
import React from "react";

interface ContextMenuProps {
  x: number;
  y: number;
  onAction: (action: string) => void;
  onClose: () => void;
}

export const ContextMenu: React.FC<ContextMenuProps> = ({
  x,
  y,
  onAction,
  onClose,
}) => {
  return (
    <div
      style={{
        position: "fixed",
        top: y,
        left: x,
        zIndex: 9999,
        backgroundColor: "#1e1e1e", // Dark theme background
        color: "#ffffff",
        border: "1px solid #333",
        borderRadius: "6px",
        boxShadow: "0 4px 12px rgba(0,0,0,0.5)",
        padding: "4px 0",
        minWidth: "160px",
        fontSize: "14px",
      }}
      // Prevent the menu itself from closing when clicked inside
      onClick={(e) => e.stopPropagation()}
    >
      <button onClick={() => onAction("rename")} style={menuItemStyle}>
        Rename
      </button>
      <div
        style={{ height: "1px", backgroundColor: "#333", margin: "4px 0" }}
      />
      <button
        onClick={() => onAction("delete")}
        style={{ ...menuItemStyle, color: "#ff4d4f" }}
      >
        Delete
      </button>
    </div>
  );
};

const menuItemStyle: React.CSSProperties = {
  width: "100%",
  textAlign: "left",
  background: "none",
  border: "none",
  padding: "8px 16px",
  cursor: "pointer",
  color: "inherit",
  fontSize: "inherit",
};
