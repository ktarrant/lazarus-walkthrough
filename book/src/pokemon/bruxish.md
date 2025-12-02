<details class="pokemon-card-container">
<summary>Bruxish (#354)</summary>
Types: Water / Psychic • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Dazzling
- Strong Jaw
- Wonder Skin *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Fighting (0.5×)
- Psychic (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM07 - Whirlpool
- TM08 - Bulk Up
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM29 - Psychic
- TM32 - Double Team
- TM33 - Reflect
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM43 - Poison Fang
- TM44 - Rest
- TM45 - Attract
- TM52 - Frost Breath
- TM56 - Scald
- HM03 - Surf
- HM07 - Waterfall

**Held Item**
Razor Fang

**Encounter Locations**
- Corrin Crossing — Fishing (20%)
- Corrin Crossing — Surfing (10%)
- Nyx Trails — Fishing (20%)
- Nyx Trails — Surfing (10%)
- Port Pello — Fishing (20%)
- Port Pello — Surfing (10%)
- Péntepetal City — Fishing (20%)
- Péntepetal City — Fishing (15%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="bruxish" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">68</span> |
| Attack | <span class="stat-value stat-high">105</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-high">92</span> |
| Total | <span class="stat-value stat-mid">480</span> |

**Level-Up Moves**
- Water Gun (Lv 1)
- Astonish (Lv 4)
- Confusion (Lv 9)
- Bite (Lv 12)
- Aqua Jet (Lv 17)
- Disable (Lv 20)
- Draining Kiss (Lv 22)
- Ice Fang (Lv 24)
- Psywave (Lv 26)
- Crunch (Lv 28)
- Covet (Lv 30)
- Aqua Tail (Lv 33)
- Screech (Lv 36)
- Psychic Fangs (Lv 39)
- Poison Fang (Lv 41)
- Synchronoise (Lv 44)

**Egg Moves**
- Water Pulse
- Poison Fang
- Ice Fang
- Rage

**Tutor Moves**
- Dream Eater
- Endure
- Icy Wind
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
