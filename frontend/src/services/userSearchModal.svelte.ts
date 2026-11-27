let isOpen = $state(false);

export const userSearchModal = {
  get isOpen() {
    return isOpen;
  },
  open() {
    isOpen = true;
  },
  close() {
    isOpen = false;
  },
};
