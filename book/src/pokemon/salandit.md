<details class="pokemon-card-container">
<summary>Salandit (#160)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-salandit">
<input type="radio" name="pokemon-tabs-salandit-group" id="pokemon-tabs-salandit-tab-0" checked>
<label for="pokemon-tabs-salandit-tab-0">Salandit</label>
<input type="radio" name="pokemon-tabs-salandit-group" id="pokemon-tabs-salandit-tab-1">
<label for="pokemon-tabs-salandit-tab-1">Salazzle</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-salandit-panel-0">
Types: Poison / Fire • Egg Groups: Monster / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Corrosion
- Oblivious *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.25×)
- Ice (0.5×)
- Fighting (0.5×)
- Poison (0.5×)
- Bug (0.25×)
- Steel (0.5×)
- Fairy (0.25×)

*Weak to*
- Water (2×)
- Ground (4×)
- Psychic (2×)
- Rock (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM20 - Poison Jab
- TM32 - Double Team
- TM35 - Flamethrower
- TM36 - Sludge Bomb
- TM38 - Fire Blast
- TM41 - Torment
- TM42 - Facade
- TM43 - Poison Fang
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM51 - Will-O-Wisp
- TM58 - Thunder Wave

**Held Item**
Smoke Ball

**Encounter Locations**
- Areios Hideout — Grass (Day) (10%)
- Areios Hideout — Grass (Night) (10%)
- Erinys Path (East) — Grass (Night) (10%)
- Fresco Isles — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="salandit" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">48</span> |
| Attack | <span class="stat-value stat-low">44</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-mid">71</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">77</span> |
| Total | <span class="stat-value stat-mid">320</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Poison Gas (Lv 1)
- Ember (Lv 5)
- Sweet Scent (Lv 8)
- Dragon Rage (Lv 13)
- Smog (Lv 16)
- Double Slap (Lv 19)
- Sludge (Lv 22)
- Flame Burst (Lv 24)
- Toxic (Lv 29)
- Nasty Plot (Lv 32)
- Fiery Wrath (Lv 35)
- Venoshock (Lv 37)
- Flamethrower (Lv 40)
- Venom Drench (Lv 45)
- Dragon Pulse (Lv 48)

**Egg Moves**
- Belch
- Knock Off
- Sand Attack
- Snatch
- Fake Out

**Tutor Moves**
- Acid Spray
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-salandit-panel-1">
Types: Poison / Fire • Egg Groups: Monster / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Corrosion
- Oblivious *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.25×)
- Ice (0.5×)
- Fighting (0.5×)
- Poison (0.5×)
- Bug (0.25×)
- Steel (0.5×)
- Fairy (0.25×)

*Weak to*
- Water (2×)
- Ground (4×)
- Psychic (2×)
- Rock (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM20 - Poison Jab
- TM32 - Double Team
- TM35 - Flamethrower
- TM36 - Sludge Bomb
- TM38 - Fire Blast
- TM41 - Torment
- TM42 - Facade
- TM43 - Poison Fang
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM51 - Will-O-Wisp
- TM58 - Thunder Wave
- TM60 - Dragon Dance

**Held Item**
Smoke Ball

**Evolution Info**
Lv. 33, Female
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="salazzle" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">68</span> |
| Attack | <span class="stat-value stat-mid">74</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-high">111</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-high">117</span> |
| Total | <span class="stat-value stat-mid">490</span> |

**Level-Up Moves**
- Captivate (Lv Evo)
- Disable (Lv 1)
- Encore (Lv 1)
- Torment (Lv 1)
- Swagger (Lv 1)
- Scratch (Lv 1)
- Poison Gas (Lv 1)
- Ember (Lv 5)
- Sweet Scent (Lv 8)
- Dragon Rage (Lv 13)
- Smog (Lv 16)
- Double Slap (Lv 19)
- Sludge (Lv 22)
- Flame Burst (Lv 24)
- Toxic (Lv 29)
- Nasty Plot (Lv 32)
- Fiery Wrath (Lv 35)
- Venoshock (Lv 37)
- Flamethrower (Lv 40)
- Venom Drench (Lv 45)
- Dragon Pulse (Lv 48)

**Egg Moves**
- Belch
- Knock Off
- Sand Attack
- Snatch
- Fake Out

**Tutor Moves**
- Acid Spray
- Body Slam
- Endure
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
#pokemon-tabs-salandit-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-salandit-panel-0 { display: block; }
#pokemon-tabs-salandit-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-salandit-panel-1 { display: block; }
</style>
</details>
