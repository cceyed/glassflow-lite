import { test, expect } from '@playwright/test';

test.describe('Architect IPC Commands', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the Tauri app
    await page.goto('http://localhost:1420');
    await page.waitForLoadState('networkidle');
  });

  test('should display the Architect Test UI', async ({ page }) => {
    // Check for main heading
    await expect(page.getByText('Architect Agent Test')).toBeVisible();
    
    // Check for input textarea
    const textarea = page.getByPlaceholder(/Build a React todo app/i);
    await expect(textarea).toBeVisible();
    
    // Check for buttons
    await expect(page.getByRole('button', { name: /Analyze Specification/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /Get State/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /Cancel/i })).toBeVisible();
  });

  test('should get initial state as idle', async ({ page }) => {
    // Click Get State button
    await page.getByRole('button', { name: /Get State/i }).click();
    
    // Wait a moment for state to update
    await page.waitForTimeout(500);
    
    // Check that state shows "idle"
    await expect(page.getByText(/State: idle/i)).toBeVisible();
  });

  test('should show error for empty specification', async ({ page }) => {
    // Click Analyze without entering spec
    await page.getByRole('button', { name: /Analyze Specification/i }).click();
    
    // Should show error
    await expect(page.getByText(/Please enter a specification/i)).toBeVisible();
  });

  test('should analyze specification and get response', async ({ page }) => {
    // Enter a specification
    const textarea = page.getByPlaceholder(/Build a React todo app/i);
    await textarea.fill('Build a simple React todo app with TypeScript');
    
    // Click Analyze
    await page.getByRole('button', { name: /Analyze Specification/i }).click();
    
    // Should show analyzing state
    await expect(page.getByText(/State: analyzing/i)).toBeVisible();
    
    // Wait for response (max 30 seconds for API call)
    await page.waitForSelector('text=/LLM Response:|Error:/i', { timeout: 30000 });
    
    // Should either show response or error
    const hasResponse = await page.getByText(/LLM Response:/i).isVisible().catch(() => false);
    const hasError = await page.getByText(/Error:/i).isVisible().catch(() => false);
    
    expect(hasResponse || hasError).toBeTruthy();
    
    // If successful, state should be complete
    if (hasResponse) {
      await expect(page.getByText(/State: complete/i)).toBeVisible();
    } else {
      // If error, state should be error
      await expect(page.getByText(/State: error/i)).toBeVisible();
    }
  });

  test('should cancel and reset to idle', async ({ page }) => {
    // Enter a specification
    const textarea = page.getByPlaceholder(/Build a React todo app/i);
    await textarea.fill('Build a simple React todo app');
    
    // Click Analyze
    await page.getByRole('button', { name: /Analyze Specification/i }).click();
    
    // Wait a moment
    await page.waitForTimeout(1000);
    
    // Click Cancel
    await page.getByRole('button', { name: /Cancel/i }).click();
    
    // Should reset to idle
    await expect(page.getByText(/State: idle/i)).toBeVisible();
    
    // Response should be cleared
    const hasResponse = await page.getByText(/LLM Response:/i).isVisible().catch(() => false);
    expect(hasResponse).toBeFalsy();
  });

  test('should handle state transitions correctly', async ({ page }) => {
    // 1. Initial state should be idle
    await page.getByRole('button', { name: /Get State/i }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText(/State: idle/i)).toBeVisible();
    
    // 2. Enter spec and analyze
    const textarea = page.getByPlaceholder(/Build a React todo app/i);
    await textarea.fill('Build a CLI tool');
    await page.getByRole('button', { name: /Analyze Specification/i }).click();
    
    // 3. Should transition to analyzing
    await expect(page.getByText(/State: analyzing/i)).toBeVisible();
    
    // 4. Wait for completion or error
    await page.waitForSelector('text=/State: (complete|error)/i', { timeout: 30000 });
    
    // 5. Cancel should reset to idle
    await page.getByRole('button', { name: /Cancel/i }).click();
    await expect(page.getByText(/State: idle/i)).toBeVisible();
  });
});
