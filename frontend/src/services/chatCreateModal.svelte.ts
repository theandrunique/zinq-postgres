let isOpen = $state(false);

function createChatCreateModal() {
  function open() {
    isOpen = true;
  }

  function close() {
    isOpen = false;
  }

  return {
    get isOpen() {
      return isOpen;
    },
    open,
    close,
  };
}

export const chatCreateModal = createChatCreateModal();
