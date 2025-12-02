<details class="pokemon-card-container">
<summary>Spiky-Eared Pichu (#248)</summary>
Types: Electric • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Static
- Lightning Rod *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- TM01 - Wish
- TM06 - Toxic
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM58 - Thunder Wave
- HM03 - Surf
- HM05 - Flash

**Evolution Info**
Can't Evolve
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="spiky-eared-pichu" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">55</span> |
| Attack | <span class="stat-value stat-mid">55</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-high">115</span> |
| Total | <span class="stat-value stat-mid">430</span> |

**Level-Up Moves**
- Relic Song (Lv 1)
- Thunder Shock (Lv 1)
- Charm (Lv 1)
- Tail Whip (Lv 5)
- Quick Attack (Lv 8)
- Sweet Kiss (Lv 10)
- Nasty Plot (Lv 13)
- Thunder Wave (Lv 15)
- Nuzzle (Lv 18)
- Volt Switch (Lv 22)
- Draining Kiss (Lv 25)
- Psychic (Lv 30)
- Discharge (Lv 33)
- Ancient Power (Lv 35)
- Dazzling Gleam (Lv 37)
- Extreme Speed (Lv 40)
- Baton Pass (Lv 42)
- Thunderclap (Lv 45)
- Thunder Cage (Lv 54)

**Egg Moves**
- Reversal
- Bide
- Present
- Encore
- Double Slap
- Wish
- Charge
- Fake Out
- Thunder Punch
- Tickle
- Flail
- Endure
- Lucky Chant
- Bestow
- Disarming Voice
- Electric Terrain

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Endure
- Mega Kick
- Mega Punch
- Mud-Slap
- Rollout
- Seismic Toss
- Sleep Talk
- Snore
- Swagger
- Swift
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
