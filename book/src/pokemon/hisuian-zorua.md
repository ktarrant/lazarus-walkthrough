<details class="pokemon-card-container">
<summary>Hisuian Zorua (#198)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-hisuian-zorua">
<input type="radio" name="pokemon-tabs-hisuian-zorua-group" id="pokemon-tabs-hisuian-zorua-tab-0" checked>
<label for="pokemon-tabs-hisuian-zorua-tab-0">Hisuian Zorua</label>
<input type="radio" name="pokemon-tabs-hisuian-zorua-group" id="pokemon-tabs-hisuian-zorua-tab-1">
<label for="pokemon-tabs-hisuian-zorua-tab-1">Hisuian Zoroark</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-hisuian-zorua-panel-0">
Types: Normal / Ghost • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Illusion
- Fluffy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)
- Ghost (0×)

*Weak to*
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM28 - Dig
- TM30 - Shadow Ball
- TM36 - Sludge Bomb
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM46 - Thief
- TM51 - Will-O-Wisp
- TM55 - Snarl
- TM59 - Dark Pulse
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-zorua" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">35</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-mid">85</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-mid">330</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Leer (Lv 1)
- Pursuit (Lv 5)
- Fake Tears (Lv 9)
- Shadow Sneak (Lv 13)
- Skitter Smack (Lv 15)
- Snarl (Lv 17)
- Scary Face (Lv 21)
- Hex (Lv 25)
- Knock Off (Lv 28)
- Will-O-Wisp (Lv 32)
- Shadow Ball (Lv 35)
- Shadow Claw (Lv 37)
- Uproar (Lv 40)
- Embargo (Lv 42)
- Infernal Parade (Lv 45)
- Nasty Plot (Lv 49)
- Extrasensory (Lv 53)
- Night Daze (Lv 57)

**Tutor Moves**
- Endure
- Icy Wind
- Psych Up
- Sleep Talk
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
<div class="pokemon-tab-panel" id="pokemon-tabs-hisuian-zorua-panel-1">
Types: Normal / Ghost • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Illusion
- Fluffy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)
- Ghost (0×)

*Weak to*
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM28 - Dig
- TM29 - Psychic
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM35 - Flamethrower
- TM36 - Sludge Bomb
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM46 - Thief
- TM51 - Will-O-Wisp
- TM55 - Snarl
- TM59 - Dark Pulse
- HM06 - Rock Smash

**Evolution Info**
Lv. 30
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-zoroark" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">55</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-high">125</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-high">110</span> |
| Total | <span class="stat-value stat-mid">520</span> |

**Level-Up Moves**
- Bitter Malice (Lv Evo)
- Shadow Sneak (Lv 1)
- Scratch (Lv 1)
- Leer (Lv 1)
- Pursuit (Lv 5)
- Fake Tears (Lv 9)
- Shadow Sneak (Lv 13)
- Skitter Smack (Lv 15)
- Snarl (Lv 17)
- Scary Face (Lv 21)
- Hex (Lv 25)
- Knock Off (Lv 28)
- Will-O-Wisp (Lv 32)
- Shadow Ball (Lv 35)
- Shadow Claw (Lv 37)
- Uproar (Lv 40)
- Embargo (Lv 42)
- Infernal Parade (Lv 45)
- Nasty Plot (Lv 49)
- Extrasensory (Lv 53)
- Night Daze (Lv 57)

**Tutor Moves**
- Body Slam
- Endure
- Icy Wind
- Psych Up
- Sleep Talk
- Swift
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
</div>
</div>
</div>
<style>
#pokemon-tabs-hisuian-zorua-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-hisuian-zorua-panel-0 { display: block; }
#pokemon-tabs-hisuian-zorua-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-hisuian-zorua-panel-1 { display: block; }
</style>
</details>
