<details class="pokemon-card-container">
<summary>Paldean Tauros-A (#224)</summary>
Types: Fighting / Water • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Anger Point
- Cud Chew *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM07 - Whirlpool
- TM08 - Bulk Up
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM28 - Dig
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM46 - Thief
- TM49 - Bulldoze
- HM03 - Surf
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="paldean-tauros-a" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-high">105</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Tail Whip (Lv 1)
- Work Up (Lv 5)
- Double Kick (Lv 10)
- Aqua Jet (Lv 15)
- Headbutt (Lv 20)
- Scary Face (Lv 25)
- Zen Headbutt (Lv 30)
- Raging Bull (Lv 35)
- Rest (Lv 40)
- Swagger (Lv 45)
- Thrash (Lv 50)
- Wave Crash (Lv 55)
- Close Combat (Lv 60)

**Egg Moves**
- Curse
- Endeavor

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Rock Slide
- Sleep Talk
- Swagger
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
