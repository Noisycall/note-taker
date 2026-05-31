import { useContextMenu } from "../hooks/useContextMenu";
import { ContextMenu } from "./ContextMenu";

const SidebarItems = () => {
  const { menuState, handleContextMenu, handleAction } = useContextMenu();

  const items = [
    { id: "item-1", label: "Project Alpha" },
    { id: "item-2", label: "Project Beta" },
    { id: "item-3", label: "Project Gamma" },
  ];

  return (
    <div style={{ padding: "16px" }}>
      {items.map((item) => (
        <div
          key={item.id}
          onContextMenu={(e) => handleContextMenu(e, item.id)}
          style={{
            padding: "8px 12px",
            cursor: "default",
            borderRadius: "4px",
            marginBottom: "4px",
            backgroundColor: "transparent",
          }}
          onMouseEnter={(e) =>
            (e.currentTarget.style.backgroundColor = "#2a2a2a")
          }
          onMouseLeave={(e) =>
            (e.currentTarget.style.backgroundColor = "transparent")
          }
        >
          {item.label}
        </div>
      ))}

      {/* Render the menu only if open */}
      {menuState.isOpen && (
        <ContextMenu
          x={menuState.x}
          y={menuState.y}
          onAction={handleAction}
          onClose={() => {}} // Handled by hook
        />
      )}
    </div>
  );
};

export default SidebarItems;
