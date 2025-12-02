<details class="pokemon-card-container">
<summary>Dracovish (#410)</summary>
Types: Water / Dragon • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Absorb
- Strong Jaw
- Drizzle *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.25×)
- Water (0.25×)
- Steel (0.5×)

*Weak to*
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- TM07 - Whirlpool
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM49 - Bulldoze
- TM56 - Scald
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dracovish" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Water Gun (Lv 1)
- Protect (Lv 7)
- Brutal Swing (Lv 12)
- Ancient Power (Lv 16)
- Bite (Lv 20)
- Dragon Breath (Lv 24)
- Ice Fang (Lv 28)
- Stomp (Lv 33)
- Super Fang (Lv 35)
- Stomping Tantrum (Lv 38)
- Slam (Lv 40)
- Crunch (Lv 44)
- Aqua Jet (Lv 48)
- Fishious Rend (Lv 52)
- Dragon Pulse (Lv 55)
- Dragon Rush (Lv 58)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Endure
- Mega Kick
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
