
import { invoke } from '@tauri-apps/api/core';

/**
 * Request an in-app review from the user.
 * This will show Google's native in-app review dialog on Android.
 * 
 * Note: Google limits how often this can be shown to users,
 * so it may not appear every time it's called.
 * 
 * @returns A promise that resolves when the review flow completes
 * 
 * @example
 * ```typescript
 * import { requestReview } from 'tauri-plugin-review-api';
 * 
 * // Request review after user completes a task
 * async function showReview() {
 *   try {
 *     await requestReview();
 *     console.log('Review flow completed');
 *   } catch (error) {
 *     console.error('Failed to show review:', error);
 *   }
 * }
 * ```
 */
export async function requestReview(): Promise<void> {
  return await invoke('plugin:review|request_review');
}
