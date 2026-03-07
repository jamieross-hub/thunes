import { create } from "zustand";
import { getContext, updateContext } from "../api";
import { Context } from "../../../cli/bindings/Context";

interface ContextState {
  context: Context;
  update: (Context: Context) => Promise<void>;
}

// TODO: generalize errors with snack bars.

export const useContextStore = create<ContextState>((set) => ({
  context: { last_opened_account: undefined },
  update: async (Context: Context) => {
    await updateContext(Context);
    const updated = await getContext();

    set({
      context: updated,
    });
  },
}));
