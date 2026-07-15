import { describe, expect, test } from 'bun:test';
import { visualizationSpecs } from '../src/lib/visualizations';
import catalog from '../../content/curriculum/catalog.json';

describe('preset algorithm visualizations', () => {
  test('ships the exact curated set without duplicate ids', () => {
    expect(visualizationSpecs).toHaveLength(25);
    expect(new Set(visualizationSpecs.map((spec) => spec.id)).size).toBe(25);
  });

  test('every animation teaches a multi-step process and ends with a memory cue', () => {
    for (const spec of visualizationSpecs) {
      expect(spec.frames.length).toBeGreaterThanOrEqual(4);
      for (const frame of spec.frames) {
        expect(frame.title.length).toBeGreaterThan(0);
        expect(frame.description.length).toBeGreaterThan(10);
        expect(frame.codeLine.length).toBeGreaterThan(0);
        expect(Boolean(frame.cells || frame.nodes || frame.table)).toBe(true);
      }
      expect(spec.frames.at(-1)?.takeaway?.startsWith('记忆点：')).toBe(true);
    }
  });

  test('matches every visualization referenced by the bundled curriculum', () => {
    const frontendIds = visualizationSpecs.map((spec) => spec.id).sort();
    const catalogIds = catalog.visualizations.map((spec) => spec.id).sort();
    expect(frontendIds).toEqual(catalogIds);
    for (const lesson of catalog.lessons) {
      if (lesson.visualization_id) expect(frontendIds).toContain(lesson.visualization_id);
    }
  });
});
