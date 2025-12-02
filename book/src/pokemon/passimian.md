<details class="pokemon-card-container">
<summary>Passimian (#347)</summary>
Types: Normal / Fighting • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Receiver
- Scrappy
- Defiant *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Bug (0.5×)
- Rock (0.5×)
- Ghost (0×)
- Dark (0.5×)

*Weak to*
- Fighting (2×)
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM34 - Shock Wave
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- HM06 - Rock Smash

**Encounter Locations**
- Kalami City — Grass (Day) (2%)
- Kaptara Island (West) — Grass (Day) (10%)
- Wakewater Isle — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="passimian" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-high">95</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Leer (Lv 4)
- Rock Smash (Lv 8)
- Focus Energy (Lv 11)
- Beat Up (Lv 15)
- Scary Face (Lv 18)
- Headbutt (Lv 20)
- Bestow (Lv 22)
- Thrash (Lv 25)
- Seed Bomb (Lv 29)
- Bulk Up (Lv 32)
- Double-Edge (Lv 36)
- Fling (Lv 38)
- Close Combat (Lv 40)
- Iron Head (Lv 43)
- Reversal (Lv 46)
- Giga Impact (Lv 50)

**Egg Moves**
- Seismic Toss
- Vital Throw
- Quick Guard
- Iron Head
- Quick Attack
- Feint

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Endure
- Mega Kick
- Mega Punch
- Rock Slide
- Seismic Toss
- Sleep Talk
- Snore
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
