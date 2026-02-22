export const greet = async (name: string = ""): Promise<string> => {
  const { invoke } = await (window as any).__TAURI__.core;
  return await invoke('greet', { name });
}
