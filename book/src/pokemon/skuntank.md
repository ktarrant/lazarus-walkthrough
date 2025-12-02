<details class="pokemon-card-container">
<summary>Skuntank (#241)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-skuntank">
<input type="radio" name="pokemon-tabs-skuntank-group" id="pokemon-tabs-skuntank-tab-0">
<label for="pokemon-tabs-skuntank-tab-0">Stunky</label>
<input type="radio" name="pokemon-tabs-skuntank-group" id="pokemon-tabs-skuntank-tab-1" checked>
<label for="pokemon-tabs-skuntank-tab-1">Skuntank</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-skuntank-panel-0">
Types: Poison / Dark • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Stench
- Aftermath
- Keen Eye *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Poison (0.5×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM23 - Hex
- TM28 - Dig
- TM30 - Shadow Ball
- TM32 - Double Team
- TM35 - Flamethrower
- TM36 - Sludge Bomb
- TM38 - Fire Blast
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM55 - Snarl
- TM59 - Dark Pulse
- HM01 - Cut
- HM06 - Rock Smash

**Encounter Locations**
- Asfal Hills — Grass (Night) (6%)
- Kipos Town — Grass (Night) (20%)
- Pythios Cemetery — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="stunky" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">68</span> |
| Attack | <span class="stat-value stat-mid">63</span> |
| Defense | <span class="stat-value stat-mid">52</span> |
| Sp. Atk | <span class="stat-value stat-low">41</span> |
| Sp. Def | <span class="stat-value stat-low">41</span> |
| Speed | <span class="stat-value stat-mid">74</span> |
| Total | <span class="stat-value stat-mid">339</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Focus Energy (Lv 1)
- Poison Gas (Lv 3)
- Screech (Lv 7)
- Fury Swipes (Lv 9)
- Smokescreen (Lv 13)
- Feint (Lv 15)
- Acid Spray (Lv 19)
- Bite (Lv 21)
- Slash (Lv 25)
- Toxic (Lv 27)
- Night Slash (Lv 31)
- Memento (Lv 33)
- Venom Drench (Lv 37)
- Sucker Punch (Lv 39)
- Belch (Lv 43)
- Explosion (Lv 45)

**Egg Moves**
- Pursuit
- Leer
- Smog
- Double-Edge
- Crunch
- Scary Face
- Astonish
- Punishment
- Haze
- Iron Tail
- Foul Play
- Flame Burst
- Play Rough

**Tutor Moves**
- Acid Spray
- Body Slam
- Double-Edge
- Endure
- Explosion
- Fury Cutter
- Mud-Slap
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
<div class="pokemon-tab-panel" id="pokemon-tabs-skuntank-panel-1">
Types: Poison / Dark • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Stench
- Aftermath
- Keen Eye *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Poison (0.5×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM23 - Hex
- TM28 - Dig
- TM30 - Shadow Ball
- TM32 - Double Team
- TM35 - Flamethrower
- TM36 - Sludge Bomb
- TM38 - Fire Blast
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM55 - Snarl
- TM59 - Dark Pulse
- HM01 - Cut
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 34

**Encounter Locations**
- Kaptara Island (East) — Grass (Night) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="skuntank" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">113</span> |
| Attack | <span class="stat-value stat-high">93</span> |
| Defense | <span class="stat-value stat-mid">67</span> |
| Sp. Atk | <span class="stat-value stat-mid">71</span> |
| Sp. Def | <span class="stat-value stat-mid">61</span> |
| Speed | <span class="stat-value stat-mid">84</span> |
| Total | <span class="stat-value stat-mid">489</span> |

**Level-Up Moves**
- Flamethrower (Lv Evo)
- Scratch (Lv 1)
- Focus Energy (Lv 1)
- Poison Gas (Lv 3)
- Screech (Lv 7)
- Fury Swipes (Lv 9)
- Smokescreen (Lv 13)
- Feint (Lv 15)
- Acid Spray (Lv 19)
- Bite (Lv 21)
- Slash (Lv 25)
- Toxic (Lv 27)
- Night Slash (Lv 31)
- Memento (Lv 33)
- Venom Drench (Lv 37)
- Sucker Punch (Lv 39)
- Belch (Lv 43)
- Explosion (Lv 45)

**Egg Moves**
- Pursuit
- Leer
- Smog
- Double-Edge
- Crunch
- Scary Face
- Astonish
- Punishment
- Haze
- Iron Tail
- Foul Play
- Flame Burst
- Play Rough

**Tutor Moves**
- Acid Spray
- Body Slam
- Double-Edge
- Endure
- Explosion
- Fury Cutter
- Mud-Slap
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
#pokemon-tabs-skuntank-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-skuntank-panel-0 { display: block; }
#pokemon-tabs-skuntank-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-skuntank-panel-1 { display: block; }
</style>
</details>
