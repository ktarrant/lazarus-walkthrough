<details class="pokemon-card-container">
<summary>Oranguru (#346)</summary>
Types: Normal / Psychic • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Inner Focus
- Telepathy
- Symbiosis *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Psychic (0.5×)
- Ghost (0×)

*Weak to*
- Bug (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM29 - Psychic
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM33 - Reflect
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM48 - Skill Swap
- TM49 - Bulldoze

**Encounter Locations**
- Kalami City — Grass (Night) (2%)
- Kaptara Island (West) — Grass (Day) (10%)
- Wakewater Isle — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="oranguru" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-mid">90</span> |
| Sp. Def | <span class="stat-value stat-high">110</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- Confusion (Lv 1)
- After You (Lv 4)
- Taunt (Lv 8)
- Quash (Lv 11)
- Stored Power (Lv 15)
- Psych Up (Lv 18)
- Leafage (Lv 20)
- Feint Attack (Lv 22)
- Nasty Plot (Lv 25)
- Magical Leaf (Lv 29)
- Instruct (Lv 32)
- Foul Play (Lv 36)
- Calm Mind (Lv 38)
- Psychic (Lv 40)
- Uproar (Lv 43)
- Future Sight (Lv 46)
- Trick Room (Lv 50)

**Egg Moves**
- Extrasensory
- Wonder Room
- Psychic Terrain

**Tutor Moves**
- Body Slam
- Dream Eater
- Endure
- Mega Kick
- Mega Punch
- Psych Up
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
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
