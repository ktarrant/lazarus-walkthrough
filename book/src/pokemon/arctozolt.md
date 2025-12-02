<details class="pokemon-card-container">
<summary>Arctozolt (#409)</summary>
Types: Electric / Ice • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Volt Absorb
- Strong Jaw
- Snow Warning *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Ice (0.5×)
- Flying (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM49 - Bulldoze
- TM58 - Thunder Wave
- HM03 - Surf
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="arctozolt" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-mid">90</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Powder Snow (Lv 1)
- Thunder Shock (Lv 1)
- Charge (Lv 7)
- Echoed Voice (Lv 12)
- Ancient Power (Lv 16)
- Aurora Beam (Lv 20)
- Pluck (Lv 24)
- Avalanche (Lv 28)
- Ice Fang (Lv 33)
- Thunder Fang (Lv 35)
- Aqua Tail (Lv 38)
- Slam (Lv 40)
- Freeze-Dry (Lv 44)
- Discharge (Lv 48)
- Bolt Beak (Lv 50)
- Icicle Crash (Lv 53)
- Blizzard (Lv 57)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Endure
- Icy Wind
- Mega Kick
- Mega Punch
- Rock Slide
- Sleep Talk
- Snore
- Thunder Punch
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
