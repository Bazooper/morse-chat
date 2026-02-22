const tauriInvoke = (window as any).__TAURI__.core.invoke;

export const greet = async (name: string = ""): Promise<string> => {
  return await tauriInvoke('greet', { name });
}
