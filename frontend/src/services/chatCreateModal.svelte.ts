import type { PublicUser } from "$lib/types";

let isOpen = $state(false);
let name = $state("");
let query = $state("");
let members = $state<PublicUser[]>([]);

function open() {
  isOpen = true;
}

function close() {
  isOpen = false;
}

function reset() {
  name = "";
  query = "";
  members = [];
}

function addMember(user: PublicUser) {
  if (members.some((member) => member.id === user.id)) return;

  members = [...members, user];
}

function removeMember(userId: string) {
  members = members.filter((member) => member.id !== userId);
}

export const chatCreateModal = {
  get isOpen() {
    return isOpen;
  },
  get name() {
    return name;
  },
  set name(value: string) {
    name = value;
  },
  get query() {
    return query;
  },
  set query(value: string) {
    query = value;
  },
  get members() {
    return members;
  },
  open,
  close,
  reset,
  addMember,
  removeMember,
};
