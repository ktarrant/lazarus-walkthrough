<details class="pokemon-card-container">
<summary>Swoobat (#110)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-swoobat">
<input type="radio" name="pokemon-tabs-swoobat-group" id="pokemon-tabs-swoobat-tab-0">
<label for="pokemon-tabs-swoobat-tab-0">Woobat</label>
<input type="radio" name="pokemon-tabs-swoobat-group" id="pokemon-tabs-swoobat-tab-1" checked>
<label for="pokemon-tabs-swoobat-tab-1">Swoobat</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-swoobat-panel-0">
Types: Psychic / Flying • Egg Groups: Field / Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Unaware
- Harvest
- Simple *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.25×)
- Ground (0×)
- Psychic (0.5×)

*Weak to*
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM12 - Taunt
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM47 - Steel Wing
- TM48 - Skill Swap
- TM57 - Roost
- TM58 - Thunder Wave
- HM02 - Fly
- HM05 - Flash

**Encounter Locations**
- Acrisia Mountains — Grass (Day) (10%)
- Acrisia Mountains — Grass (Night) (10%)
- Jusmail Town — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="woobat" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-low">43</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-low">43</span> |
| Speed | <span class="stat-value stat-mid">72</span> |
| Total | <span class="stat-value stat-mid">328</span> |

**Level-Up Moves**
- Confusion (Lv 1)
- Odor Sleuth (Lv 4)
- Gust (Lv 8)
- Assurance (Lv 12)
- Heart Stamp (Lv 15)
- Mega Drain (Lv 18)
- Imprison (Lv 19)
- Air Cutter (Lv 21)
- Attract (Lv 25)
- Amnesia (Lv 29)
- Calm Mind (Lv 29)
- Air Slash (Lv 32)
- Future Sight (Lv 36)
- Psychic (Lv 39)
- Giga Drain (Lv 42)
- Endeavor (Lv 47)

**Egg Moves**
- Charm
- Knock Off
- Fake Tears
- Supersonic
- Synchronoise
- Stored Power
- Roost
- Flatter
- Helping Hand
- Captivate
- Venom Drench
- Psycho Shift

**Tutor Moves**
- Dream Eater
- Endure
- Psych Up
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
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-swoobat-panel-1">
Types: Psychic / Flying • Egg Groups: Field / Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Unaware
- Harvest
- Simple *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.25×)
- Ground (0×)
- Psychic (0.5×)

*Weak to*
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM12 - Taunt
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM47 - Steel Wing
- TM48 - Skill Swap
- TM57 - Roost
- TM58 - Thunder Wave
- HM02 - Fly
- HM05 - Flash

**Evolution Info**
Lv. 24
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="swoobat" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">77</span> |
| Attack | <span class="stat-value stat-mid">67</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-mid">87</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-high">114</span> |
| Total | <span class="stat-value stat-mid">460</span> |

**Level-Up Moves**
- Leech Life (Lv Evo)
- Confusion (Lv 1)
- Odor Sleuth (Lv 4)
- Gust (Lv 8)
- Assurance (Lv 12)
- Heart Stamp (Lv 15)
- Mega Drain (Lv 18)
- Imprison (Lv 19)
- Air Cutter (Lv 21)
- Attract (Lv 25)
- Amnesia (Lv 29)
- Calm Mind (Lv 29)
- Air Slash (Lv 32)
- Future Sight (Lv 36)
- Psychic (Lv 39)
- Giga Drain (Lv 42)
- Endeavor (Lv 47)

**Egg Moves**
- Charm
- Knock Off
- Fake Tears
- Supersonic
- Synchronoise
- Stored Power
- Roost
- Flatter
- Helping Hand
- Captivate
- Venom Drench
- Psycho Shift

**Tutor Moves**
- Dream Eater
- Endure
- Psych Up
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
</div>
</div>
</div>
<style>
#pokemon-tabs-swoobat-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-swoobat-panel-0 { display: block; }
#pokemon-tabs-swoobat-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-swoobat-panel-1 { display: block; }
</style>
</details>
