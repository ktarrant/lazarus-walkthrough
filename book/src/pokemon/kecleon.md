<details class="pokemon-card-container">
<summary>Kecleon (#341)</summary>
Types: Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Color Change
- Protean
- Prism Armor *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Ghost (0×)

*Weak to*
- Fighting (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM24 - Thunderbolt
- TM25 - Thunder
- TM28 - Dig
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM34 - Shock Wave
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM53 - Power-Up Punch
- TM58 - Thunder Wave
- HM01 - Cut
- HM04 - Strength
- HM05 - Flash
- HM06 - Rock Smash

**Encounter Locations**
- Sea of Vulcai — Grass (Day) (10%)
- Sea of Vulcai — Grass (Night) (10%)
- Tower of Dioxippus — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="kecleon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-high">120</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-mid">450</span> |

**Level-Up Moves**
- Thief (Lv 1)
- Tail Whip (Lv 1)
- Astonish (Lv 1)
- Lick (Lv 1)
- Scratch (Lv 1)
- Bind (Lv 4)
- Shadow Sneak (Lv 7)
- Feint (Lv 10)
- Fury Swipes (Lv 13)
- Feint Attack (Lv 16)
- Psybeam (Lv 18)
- Ancient Power (Lv 21)
- Force Palm (Lv 23)
- Slash (Lv 25)
- Glare (Lv 28)
- Camouflage (Lv 30)
- Slack Off (Lv 31)
- Shadow Claw (Lv 33)
- Drain Punch (Lv 36)
- Screech (Lv 38)
- Iron Head (Lv 40)
- Substitute (Lv 42)
- Body Slam (Lv 44)
- Sucker Punch (Lv 46)
- Synchronoise (Lv 50)

**Egg Moves**
- Disable
- Magic Coat
- Trick
- Fake Out
- Nasty Plot
- Dizzy Punch
- Recover
- Skill Swap
- Snatch
- Foul Play
- Camouflage
- Power-Up Punch

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Dynamic Punch
- Endure
- Fire Punch
- Fury Cutter
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Metronome
- Mud-Slap
- Psych Up
- Rock Slide
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
