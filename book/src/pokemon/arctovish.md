<details class="pokemon-card-container">
<summary>Arctovish (#411)</summary>
Types: Water / Ice • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Absorb
- Strong Jaw
- Snow Warning *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Ice (0.25×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)
- Rock (2×)

**TM/HM Moves**
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="arctovish" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Powder Snow (Lv 1)
- Water Gun (Lv 1)
- Protect (Lv 7)
- Icy Wind (Lv 12)
- Ancient Power (Lv 16)
- Bite (Lv 20)
- Aurora Veil (Lv 24)
- Freeze-Dry (Lv 28)
- Ice Shard (Lv 33)
- Super Fang (Lv 35)
- Crunch (Lv 38)
- Slam (Lv 40)
- Aqua Jet (Lv 44)
- Wave Crash (Lv 48)
- Fishious Rend (Lv 52)
- Icicle Crash (Lv 55)
- Blizzard (Lv 58)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Endure
- Icy Wind
- Rock Slide
- Sleep Talk
- Snore
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
