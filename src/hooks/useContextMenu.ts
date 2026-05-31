// src/hooks/useContextMenu.ts
import React, { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core"; // Adjust import path for your Tauri version

interface MenuState {
  isOpen: boolean;
  x: number;
  y: number;
  targetId: string | null;
}

export const useContextMenu = (onActionDone: () => void) => {
  const [menuState, setMenuState] = useState<MenuState>({
    isOpen: false,
    x: 0,
    y: 0,
    targetId: null,
  });

  // Handle the right-click event
  const handleContextMenu = useCallback((e: React.MouseEvent, id: string) => {
    e.preventDefault(); // Prevent default browser context menu

    // Calculate position, ensuring it stays within viewport bounds
    const menuWidth = 200; // Approximate width
    const menuHeight = 150; // Approximate height
    const viewportWidth = window.innerWidth;
    const viewportHeight = window.innerHeight;

    let x = e.clientX;
    let y = e.clientY;

    // Adjust if menu would overflow right
    if (x + menuWidth > viewportWidth) {
      x = viewportWidth - menuWidth - 10;
    }
    // Adjust if menu would overflow bottom
    if (y + menuHeight > viewportHeight) {
      y = viewportHeight - menuHeight - 10;
    }

    setMenuState({ isOpen: true, x, y, targetId: id });
  }, []);

  // Close the menu
  const closeMenu = useCallback(() => {
    setMenuState((prev) => ({ ...prev, isOpen: false }));
  }, []);

  // Close on click outside
  useEffect(() => {
    const handleClickOutside = (_: MouseEvent) => {
      if (menuState.isOpen) {
        closeMenu();
      }
    };

    if (menuState.isOpen) {
      window.addEventListener("click", handleClickOutside);
    }

    return () => {
      window.removeEventListener("click", handleClickOutside);
    };
  }, [menuState.isOpen, closeMenu]);

  // Close on Escape key
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape" && menuState.isOpen) {
        closeMenu();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [menuState.isOpen, closeMenu]);

  // Action handler for menu items
  const handleAction = useCallback(
    async (action: string) => {
      if (!menuState.targetId) return;

      try {
        // Example: Call a Tauri command
        if (action === "delete") {
          await invoke("delete_file", { path: menuState.targetId });
        } else if (action === "rename") {
          await invoke("rename_sidebar_item", {
            id: menuState.targetId,
            newName: "New Name",
          });
        }

        closeMenu();
      } catch (error) {
        console.error("Action failed:", error);
        // Optional: Show a toast notification here
      }
      onActionDone();
    },
    [menuState.targetId, closeMenu],
  );

  return {
    menuState,
    handleContextMenu,
    handleAction,
    closeMenu,
  };
};
