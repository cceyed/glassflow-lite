// E2E test for error handling
import { test, expect } from '@playwright/test';

test.describe('Architect Agent - Error Handling', () => {
  test('invalid spec shows error and allows retry', async ({ page }) => {
    // TODO: Implement when Tauri app is ready
    throw new Error('E2E test not implemented - Tauri app not running yet');
  });
});
