<details class="pokemon-card-container">
<summary>Own Tempo Rockruff (#105)</summary>
Types: Rock • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Own Tempo

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.5×)
- Poison (0.5×)
- Flying (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Fighting (2×)
- Ground (2×)
- Steel (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM12 - Taunt
- TM17 - Protect
- TM28 - Dig
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM55 - Snarl
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="own-tempo-rockruff" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-low">280</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Leer (Lv 1)
- Sand Attack (Lv 4)
- Bite (Lv 7)
- Howl (Lv 12)
- Rock Throw (Lv 15)
- Odor Sleuth (Lv 18)
- Rock Tomb (Lv 23)
- Roar (Lv 26)
- Stealth Rock (Lv 29)
- Rock Slide (Lv 34)
- Scary Face (Lv 37)
- Crunch (Lv 40)
- Rock Climb (Lv 45)
- Stone Edge (Lv 48)

**Egg Moves**
- Crush Claw
- Fire Fang
- Thunder Fang
- Sucker Punch
- Thrash

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Mud-Slap
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swords Dance
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
