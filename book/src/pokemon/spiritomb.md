<details class="pokemon-card-container">
<summary>Spiritomb (#230)</summary>
Types: Ghost / Dark • Egg Groups: Amorphous

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Pressure
- Infiltrator
- Prankster *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fighting (0×)
- Poison (0.5×)
- Psychic (0×)

*Weak to*
- Fairy (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM34 - Shock Wave
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM51 - Will-O-Wisp
- TM55 - Snarl
- TM59 - Dark Pulse
- HM05 - Flash
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="spiritomb" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-high">92</span> |
| Defense | <span class="stat-value stat-high">108</span> |
| Sp. Atk | <span class="stat-value stat-high">92</span> |
| Sp. Def | <span class="stat-value stat-high">108</span> |
| Speed | <span class="stat-value stat-low">35</span> |
| Total | <span class="stat-value stat-mid">495</span> |

**Level-Up Moves**
- Curse (Lv 1)
- Pursuit (Lv 1)
- Confuse Ray (Lv 1)
- Spite (Lv 1)
- Shadow Sneak (Lv 1)
- Feint Attack (Lv 7)
- Hypnosis (Lv 13)
- Dream Eater (Lv 19)
- Ominous Wind (Lv 23)
- Crunch (Lv 27)
- Sucker Punch (Lv 31)
- Psybeam (Lv 34)
- Nasty Plot (Lv 37)
- Fiery Wrath (Lv 40)
- Memento (Lv 43)
- Dark Pulse (Lv 45)
- Foul Play (Lv 48)
- Astral Barrage (Lv 55)

**Egg Moves**
- Destiny Bond
- Pain Split
- Smokescreen
- Imprison
- Grudge
- Shadow Sneak
- Captivate
- Nightmare
- Foul Play
- Disable

**Tutor Moves**
- Body Slam
- Dream Eater
- Endure
- Icy Wind
- Psych Up
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
