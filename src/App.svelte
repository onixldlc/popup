<script>
	export let question="什么";
	export let question_context="what is the pinyin of the character above:"
	export let answer="what";
	let user_answer="";

	function check_answer(){
		if(user_answer == answer){
			alert(`well done :)`)
		}else{
			alert(`${question} = ${answer}`)
		}
	}

</script>

<main>
	<h1 class="disableCopy">{question}</h1>
	<h3>{question_context}</h3>
	<input on:keydown={(e)=>{if(e.keyCode==13){check_answer()}}} bind:value={user_answer} type="text"/>
</main>

<style>
	.disableCopy{
		pointer-events: none;
		user-select:none;
	}

	input{
		transition-duration: 200ms;
		text-align: center;
	}

	input:hover{
		border: 1px solid rgb(0, 0, 0) ;
		border-radius: 3px;
	}

	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>