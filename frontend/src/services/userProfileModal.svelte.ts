let isOpen = $state(false);
let userId = $state<string | null>(null);

export const userProfileModal = {
  get isOpen() {
    return isOpen;
  },
  get userId() {
    return userId;
  },
  open(id: string) {
    userId = id;
    isOpen = true;
  },
  close() {
    isOpen = false;
  },
};
