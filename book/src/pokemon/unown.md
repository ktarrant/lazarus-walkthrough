<details class="pokemon-card-container">
<summary>Unown (#403)</summary>
Types: Psychic • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Levitate

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Psychic (0.5×)

*Weak to*
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="unown" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">58</span> |
| Attack | <span class="stat-value stat-mid">82</span> |
| Defense | <span class="stat-value stat-mid">58</span> |
| Sp. Atk | <span class="stat-value stat-mid">82</span> |
| Sp. Def | <span class="stat-value stat-mid">58</span> |
| Speed | <span class="stat-value stat-mid">58</span> |
| Total | <span class="stat-value stat-mid">396</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=hidden-power">Hidden Power</a> (Lv 1)
- <a href="move-lookup.html?q=freezing-glare">Freezing Glare</a> (Lv 30)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</details>
