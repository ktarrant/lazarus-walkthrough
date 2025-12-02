<details class="pokemon-card-container">
<summary>Komala (#181)</summary>
Types: Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Comatose
- Cheek Pouch *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Ghost (0×)

*Weak to*
- Fighting (2×)

**TM/HM Moves**
- TM01 - Wish
- TM04 - Calm Mind
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze

**Encounter Locations**
- Erinys Path (West) — Grass (Day) (10%)
- Wakewater Isle — Grass (Day) (4%)
- Wakewater Isle — Grass (Night) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="komala" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-high">95</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- Defense Curl (Lv 1)
- Rollout (Lv 1)
- Stockpile (Lv 6)
- Spit Up (Lv 6)
- Swallow (Lv 6)
- Rapid Spin (Lv 11)
- Yawn (Lv 16)
- Slam (Lv 21)
- Bulldoze (Lv 25)
- Flail (Lv 28)
- Rock Smash (Lv 30)
- Sucker Punch (Lv 31)
- Body Slam (Lv 35)
- Wood Hammer (Lv 38)
- Psych Up (Lv 43)
- Play Rough (Lv 45)

**Egg Moves**
- Charm
- Wish
- Play Rough
- Sing

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Psych Up
- Rock Slide
- Rollout
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
